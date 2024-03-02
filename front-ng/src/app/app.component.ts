import {Component,Input, ViewContainerRef} from '@angular/core';
import { NavbarComponent } from './navbar/navbar.component';
import { FileGridComponent } from './file-grid/file-grid.component';

import { Router, RouterModule } from '@angular/router';
import { GalleriesComponent } from './media/galleries.component';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    imports: [
        
        RouterModule,GalleriesComponent
      ],
    standalone: true,
})

export class AppComponent {
    constructor(private router: Router) {}
}

