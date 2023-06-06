import { fakeAsync, ComponentFixture, TestBed } from '@angular/core/testing';
import { SprintAddComponent } from './sprint-add.component';

describe('SprintAddComponent', () => {
  let component: SprintAddComponent;
  let fixture: ComponentFixture<SprintAddComponent>;

  beforeEach(fakeAsync(() => {
    TestBed.configureTestingModule({
      declarations: [ SprintAddComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SprintAddComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should compile', () => {
    expect(component).toBeTruthy();
  });
});
