/*!
 * @license
 * Copyright Google LLC All Rights Reserved.
 *
 * Use of this source code is governed by an MIT-style license that can be
 * found in the LICENSE file at https://angular.dev/license
 */

import {bootstrapApplication} from '@angular/platform-browser';
import {AppComponent} from './app/app.component';
import { provideHttpClient } from '@angular/common/http';
import { WINDOW } from './app/page.service';

bootstrapApplication(AppComponent,
    {
        providers: [
            provideHttpClient(),
        {
            provide: WINDOW, useFactory: () => window
        }, 
        ],
      }
  ).catch(err => console.error(err));
