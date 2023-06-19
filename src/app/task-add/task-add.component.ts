import {Component, OnInit} from '@angular/core';
import {UntypedFormBuilder, UntypedFormGroup, Validators} from '@angular/forms';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {Result} from "../common";
import {NzSelectOptionInterface} from "ng-zorro-antd/select/select.types";
import {Sprint} from "../sprints/sprints.component";
import {User} from "../users/users.component";
import {Observable} from "rxjs";

@Component({
  selector: 'app-task-add',
  templateUrl: './task-add.component.html',

  styleUrls: ['./task-add.component.css']
})
export class TaskAddComponent implements OnInit {
  validateForm!: UntypedFormGroup;
  sprintOptions: NzSelectOptionInterface[] = []
  developerOptions: NzSelectOptionInterface[] = []
  testOptions: NzSelectOptionInterface[] = []

  submitForm(): void {
    if (this.validateForm.valid) {
      let token = localStorage.getItem('token');
      if (!token) {
        location.replace('/login');
        return
      }
      this.http.post<Result<any>>(environment.urlPrefix + "task", this.validateForm.value, {
        headers: {'Authorization': 'Bearer ' + token}
      }).subscribe(result => {
        if (result.result_code === 200) {
          location.replace('/tasks')
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

  loadSprints(): void {
    let token = localStorage.getItem('token');
    if (!token) {
      location.replace('/login');
      return
    }
    this.http.get<Result<Sprint[]>>(environment.urlPrefix + "sprint/list", {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.sprintOptions = result.data.map(v => {
          return {
            label: v.name,
            value: v.id,
          }
        })
      }
    })
  }

  loadUsers(role: string): Observable<Result<User[]>> | null {
    let token = localStorage.getItem('token');
    if (!token) {
      location.replace('/login');
      return null;
    }
    return this.http.get<Result<User[]>>(environment.urlPrefix + "user/list", {
      headers: {'Authorization': 'Bearer ' + token},
      params: {role: role}
    })
  }

  constructor(private fb: UntypedFormBuilder, private http: HttpClient) {
  }

  ngOnInit(): void {
    this.loadSprints()
    this.loadUsers('dev')?.subscribe(result => {
      if (result.result_code === 200) {
        this.developerOptions = result.data.map(v => {
          return {
            label: v.name,
            value: v.id,
          }
        })
      }
    })
    this.loadUsers('test')?.subscribe(result => {
      if (result.result_code === 200) {
        this.testOptions = result.data.map(v => {
          return {
            label: v.name,
            value: v.id,
          }
        })
      }
    })
    this.validateForm = this.fb.group({
      name: [null, [Validators.required]],
      sprint: [null, [Validators.required]],
      ordinal: [0, [Validators.required]],
      developer: [null, [Validators.required]],
      sp: [null, [Validators.required]],
      tester: [null, []],
      test_sp: [null, []],
      // start: [null, []],
      // end: [null, []],
      // test_start: [null, []],
      // test_end: [null, []],
    });
  }
}
