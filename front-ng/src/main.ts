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
import { MediaViewerComponent } from "./app/media-viewer/media-viewer.component";

export const routes: Routes = [
    {path: '',component: GalleryGridComponent},
  { path: "gallery/:id", component: GalleryComponent },
  { path: "viewer/:galleryId/:startMediaId", component: MediaViewerComponent },
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
