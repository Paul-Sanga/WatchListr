import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';

const routes: Routes = [
  {
    path: "",
    loadComponent: () => import("./components/home/home.component").then(component => component.HomeComponent)
  },
  {
    path: "add-show",
    loadComponent: () => import("./components/add-show/add-show.component").then(component => component.AddShowComponent)
  }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
