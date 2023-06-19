import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SprintDetailComponent } from './sprint-detail.component';

describe('TasksComponent', () => {
  let component: SprintDetailComponent;
  let fixture: ComponentFixture<SprintDetailComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [SprintDetailComponent]
    });
    fixture = TestBed.createComponent(SprintDetailComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
