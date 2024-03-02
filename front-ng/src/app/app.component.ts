import {Component,Input, ViewContainerRef} from '@angular/core';
import { NavbarComponent } from './navbar/navbar.component';
import { FileGridComponent } from './file-grid/file-grid.component';

import { Router, RouterModule } from '@angular/router';
import { GalleryComponent } from './gallery/gallery.component';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    imports: [
        
        RouterModule,GalleryComponent
      ],
    standalone: true,
})

export class AppComponent {
    constructor(private router: Router) {}
}

