import {NgModule} from '@angular/core';
import {RouterModule, Routes} from '@angular/router';
import {LoginComponent} from "./login/login.component";
import {UsersComponent} from "./users/users.component";
import {UserAddComponent} from "./user-add/user-add.component";
import {SprintsComponent} from "./sprints/sprints.component";
import {SprintAddComponent} from "./sprint-add/sprint-add.component";
import {SprintDetailComponent} from "./sprint-detail/sprint-detail.component";
import {TaskAddComponent} from "./task-add/task-add.component";

const routes: Routes = [
  {path: '', pathMatch: 'full', redirectTo: '/login'},
  {path: 'login', component: LoginComponent},
  {path: 'users', component: UsersComponent},
  {path: 'user/add', component: UserAddComponent},
  {path: 'sprints', component: SprintsComponent},
  {path: 'sprint/add', component: SprintAddComponent},
  {path: 'sprint/detail/:id', component: SprintDetailComponent},
  {path: 'task/add', component: TaskAddComponent},
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
