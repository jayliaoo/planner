import {Component, OnInit} from '@angular/core';
import {UntypedFormBuilder, UntypedFormGroup, Validators} from '@angular/forms';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {Result} from "../common";

@Component({
  selector: 'app-sprint-add',
  templateUrl: './sprint-add.component.html',

  styleUrls: ['./sprint-add.component.css']
})
export class SprintAddComponent implements OnInit {
  validateForm!: UntypedFormGroup;

  submitForm(): void {
    if (this.validateForm.valid) {
      let token = localStorage.getItem('token');
      if (!token) {
        location.replace('/login');
        return
      }
      let value = this.validateForm.value;
      (value['start'] as Date).setUTCHours(0, 0, 0, 0);
      (value['end'] as Date).setUTCHours(0, 0, 0, 0);
      this.http.post<Result<any>>(environment.urlPrefix + "sprint", value, {
        headers: {'Authorization': 'Bearer ' + token}
      }).subscribe(result => {
        if (result.result_code === 200) {
          location.replace('/sprints')
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
      start: [null, [Validators.required]],
      end: [null, [Validators.required]],
    });
  }
}
