import {
  Component,
  OnInit,
} from '@angular/core'
import { HttpClient } from '@angular/common/http'
import { environment } from '../../environments/environment'
import { Result } from '../common'
import {
  UntypedFormBuilder,
  UntypedFormGroup,
} from '@angular/forms'
import { NzSelectOptionInterface } from 'ng-zorro-antd/select/select.types'
import { ActivatedRoute } from '@angular/router'
import { Sprint } from '../sprints/sprints.component'

interface Task {
  id: number
  name: string
  sprint: number
  sprint_name: string
  developer: number
  developer_name: string
  sp: number
  tester: number
  tester_name: string
  test_sp: number
  start: string
  end: string
  test_start: string
  test_end: string
}

@Component({
  selector: 'app-tasks',
  templateUrl: './sprint-detail.component.html',
  styleUrls: ['./sprint-detail.component.css'],
})
export class SprintDetailComponent implements OnInit {
  validateForm!: UntypedFormGroup
  sprint?: Sprint
  editCache: { [key: string]: { edit: boolean, data: Task } } = {}
  listOfData: Task[] = []

  constructor(private fb: UntypedFormBuilder, private http: HttpClient, private route: ActivatedRoute) {
  }

  loadTasks(): void {
    let token = localStorage.getItem('token')
    if (!token) {
      location.replace('/login')
      return
    }
    this.http.get<Result<Task[]>>(environment.urlPrefix + 'task/list', {
      headers: { 'Authorization': 'Bearer ' + token },
      params: this.validateForm.value,
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.listOfData = result.data
        this.updateEditCache()
      }
    })
  }

  startEdit(id: number): void {
    this.editCache[id.toString()].edit = true
  }

  delete(id: number): void {
    delete this.editCache[id.toString()]

    let token = localStorage.getItem('token')
    if (!token) {
      location.replace('/login')
      return
    }
    this.http.delete<Result<any>>(environment.urlPrefix + 'task/' + id, {
      headers: { 'Authorization': 'Bearer ' + token },
    }).subscribe(result => {
      if (result.result_code === 200) {
        location.replace('/tasks')
      }
    })
  }

  updateEditCache(): void {
    this.listOfData.forEach(item => {
      this.editCache[item.id.toString()] = {
        edit: false,
        data: { ...item },
      }
    })
  }

  ngOnInit(): void {
    // this.validateForm = this.fb.group({
    //   sprint: [null, []],
    // })

    let token = localStorage.getItem('token')
    if (!token) {
      location.replace('/login')
      return
    }
    let id = this.route.snapshot.paramMap.get('id') as string;

    this.http.get<Result<Sprint>>(environment.urlPrefix + 'sprint/'+id, {
      headers: { 'Authorization': 'Bearer ' + token }
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.sprint = result.data
      }

    })
    this.http.get<Result<Task[]>>(environment.urlPrefix + 'task/list', {
      headers: { 'Authorization': 'Bearer ' + token },
      params: { sprint: id },
    }).subscribe(result => {
      if (result.result_code === 200) {
        this.listOfData = result.data
        this.updateEditCache()
      }
    })
  }
}
