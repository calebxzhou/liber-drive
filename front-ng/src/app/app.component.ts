import {Component,Input, ViewContainerRef} from '@angular/core'; 

import { Router, RouterLink, RouterLinkActive, RouterModule, RouterOutlet } from '@angular/router';
import { GalleryGridComponent } from './gallery-grid/gallery-grid.component';
@Component({
    selector: 'app-root',
    templateUrl: './app.component.html',
    imports: [

        RouterModule,GalleryGridComponent, RouterOutlet, RouterLink, RouterLinkActive
      ],
    standalone: true,
})

export class AppComponent {
    constructor(private router: Router) {}
}

