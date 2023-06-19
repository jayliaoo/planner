import {Component, OnInit} from '@angular/core';
import {UntypedFormBuilder, UntypedFormGroup, Validators} from '@angular/forms';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {Result} from "../common";

@Component({
  selector: 'app-user-add',
  templateUrl: './user-add.component.html',

  styleUrls: ['./user-add.component.css']
})
export class UserAddComponent implements OnInit {
  validateForm!: UntypedFormGroup;

  submitForm(): void {
    if (this.validateForm.valid) {
      let token = localStorage.getItem('token');
      if (!token) {
        location.replace('/login');
        return
      }
      this.http.post<Result<any>>(environment.urlPrefix + "user", this.validateForm.value, {
        headers: {'Authorization': 'Bearer ' + token}
      }).subscribe(result => {
        if (result.result_code === 200) {
          location.replace('/users')
        }
      })
    } else {
      Object.values(this.validateForm.controls).forEach(control => {
        if (control.invalid) {
          control.markAsDirty();
          control.updateValueAndValidity({onlySelf: true});
        }
      });
    }
  }

  constructor(private fb: UntypedFormBuilder, private http: HttpClient) {
  }

  ngOnInit(): void {
    this.validateForm = this.fb.group({
      name: [null, [Validators.required]],
      role: [null, [Validators.required]],
    });
  }
}
