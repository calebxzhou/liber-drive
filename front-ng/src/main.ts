/*!
 * @license
 * Copyright Google LLC All Rights Reserved.
 *
 * Use of this source code is governed by an MIT-style license that can be
 * found in the LICENSE file at https://angular.dev/license
 */

import { bootstrapApplication } from "@angular/platform-browser";
import { AppComponent } from "./app/app.component";
import { provideHttpClient } from "@angular/common/http";
import { WINDOW } from "./app/page.service";
import {
  Routes,
  provideRouter,
  withComponentInputBinding,
} from "@angular/router";
import { provideAnimations } from "@angular/platform-browser/animations";
import { HomeComponent } from "./app/home/home.component";
import { AlbumComponent } from "./app/album/album.component";
import { register as registerSwiperElements } from "swiper/element/bundle";
import { LAZYLOAD_IMAGE_HOOKS, ScrollHooks } from "ng-lazyload-image";
export const routes: Routes = [
  { path: "home", component: HomeComponent },
  {
    path: ":albumName/:test",
    component: AlbumComponent,
  },
  { path: ":albumName", component: AlbumComponent },
  { path: "", redirectTo: "/home", pathMatch: "full" },
  { path: "**", redirectTo: "/home", pathMatch: "full" },
];
registerSwiperElements();
bootstrapApplication(AppComponent, {
  providers: [
    provideHttpClient(),
    {
      provide: WINDOW,
      useFactory: () => window,
    },
    provideRouter(routes, withComponentInputBinding()),
    provideAnimations(),
    { provide: LAZYLOAD_IMAGE_HOOKS, useClass: ScrollHooks },
  ],
}).catch((err) => console.error(err));
