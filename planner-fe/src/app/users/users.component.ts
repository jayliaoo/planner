import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {Result} from "../common";

export interface User {
  id: number;
  name: string;
  role: string;
}

@Component({
  selector: 'app-users',
  templateUrl: './users.component.html',
  styleUrls: ['./users.component.css']
})
export class UsersComponent implements OnInit {
  editCache: { [key: string]: { edit: boolean; data: User } } = {};
  listOfData: User[] = [];

  constructor(private http: HttpClient) {
  }

  startEdit(id: number): void {
    this.editCache[id.toString()].edit = true;
  }

  cancelEdit(id: number): void {
    const index = this.listOfData.findIndex(item => item.id === id);
    this.editCache[id.toString()] = {
      data: {...this.listOfData[index]},
      edit: false
    };
  }

  saveEdit(id: number): void {
    const index = this.listOfData.findIndex(item => item.id === id);
    Object.assign(this.listOfData[index], this.editCache[id.toString()].data);
    this.editCache[id.toString()].edit = false;

    let token = localStorage.getItem('token');
    if (!token) {
      location.replace('/login');
      return
    }
    this.http.put<Result<any>>(environment.urlPrefix + "user", this.listOfData[index], {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        location.replace('/users')
      }
    })
  }

  delete(id: number): void {
    delete this.editCache[id.toString()]

    let token = localStorage.getItem('token');
    if (!token) {
      location.replace('/login');
      return
    }
    this.http.delete<Result<any>>(environment.urlPrefix + "user/" + id, {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        location.replace('/users')
      }
    })
  }

  updateEditCache(): void {
    this.listOfData.forEach(item => {
      this.editCache[item.id.toString()] = {
        edit: false,
        data: {...item}
      };
    });
  }

  ngOnInit(): void {
    let token = localStorage.getItem('token');
    if (!token) {
      location.replace('/login');
      return
    }
    this.http.get<Result<User[]>>(environment.urlPrefix + "user/list", {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.listOfData = result.data
        this.updateEditCache();
      }
    });
  }
}
