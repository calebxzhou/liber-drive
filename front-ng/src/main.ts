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
import { MediaViewerComponent } from "./app/media-viewer/media-viewer.component";
import { provideAnimations } from "@angular/platform-browser/animations";
import { HomeComponent } from "./app/home/home.component";
import { AlbumGridComponent } from "./app/album-grid/album-grid.component";
import { AlbumComponent } from "./app/album/album.component";
import { GalleryComponent } from "./app/gallery/gallery.component";
export const routes: Routes = [
  { path: "home", component: HomeComponent},
  { path: "gallery/:galleryName/:albumName", component: AlbumComponent },
  { path: "gallery/:galleryName", component: GalleryComponent },
  { path: "viewer/:galleryId/:startMediaId", component: MediaViewerComponent },
  { path: '',   redirectTo: '/home', pathMatch: 'full' },
  { path: '**',   redirectTo: '/home', pathMatch: 'full' },
];
bootstrapApplication(AppComponent, {
  providers: [
    provideHttpClient(),
    {
      provide: WINDOW,
      useFactory: () => window,
    },
    provideRouter(routes, withComponentInputBinding()),
    provideAnimations(),
  ],
}).catch((err) => console.error(err));
