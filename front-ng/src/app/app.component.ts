import {
  CUSTOM_ELEMENTS_SCHEMA,
  Component,
  Input,
  ViewContainerRef,
} from "@angular/core";
import {
  Router,
  RouterLink,
  RouterLinkActive,
  RouterModule,
  RouterOutlet,
} from "@angular/router";
import { NavbarComponent } from "./navbar/navbar.component";
import { GalleryComponent } from "./gallery/gallery.component";
import { SwiperOptions } from "swiper/types/swiper-options";
@Component({
  selector: "app-root",
  templateUrl: "./app.component.html",
  standalone: true,
  imports: [
    RouterModule,
    RouterOutlet,
    RouterLink,
    RouterLinkActive,
    NavbarComponent,
    GalleryComponent,
  ],
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
})
export class AppComponent {
  constructor(private router: Router) {}
}
