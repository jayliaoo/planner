import {Component, OnInit} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {environment} from "../../environments/environment";
import {Result} from "../common";

interface Sprint {
  id: number;
  name: string;
  start: string | Date;
  end: string | Date;
}

@Component({
  selector: 'app-sprints',
  templateUrl: './sprints.component.html',
  styleUrls: ['./sprints.component.css']
})
export class SprintsComponent implements OnInit {
  editCache: { [key: string]: { edit: boolean; data: Sprint } } = {};
  listOfData: Sprint[] = [];

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
    let sprint = this.listOfData[index];
    if (sprint.start instanceof Date) {
      sprint.start.setUTCHours(0,0,0,0)
    }
    if (sprint.end instanceof Date) {
      sprint.end.setUTCHours(0, 0, 0, 0)
    }
    this.http.put<Result<any>>(environment.urlPrefix + "sprint", sprint, {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        location.replace('/sprints')
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
    this.http.delete<Result<any>>(environment.urlPrefix + "sprint/" + id, {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        location.replace('/sprints')
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
    this.http.get<Result<Sprint[]>>(environment.urlPrefix + "sprint/list", {
      headers: {'Authorization': 'Bearer ' + token}
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.listOfData = result.data
        this.updateEditCache();
      }
    });
  }
}
