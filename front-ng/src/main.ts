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
import { Routes, provideRouter, withComponentInputBinding } from "@angular/router"; 
import { GalleryComponent } from "./app/gallery/gallery.component";
import { GalleryGridComponent } from "./app/gallery-grid/gallery-grid.component";

export const routes: Routes = [
    {path: '',component: GalleryGridComponent},
  { path: "gallery/:name", component: GalleryComponent },
];
bootstrapApplication(AppComponent, {
  providers: [
    provideHttpClient(),
    {
      provide: WINDOW,
      useFactory: () => window,
    },
    provideRouter(routes, withComponentInputBinding()),
  ],
}).catch((err) => console.error(err));
