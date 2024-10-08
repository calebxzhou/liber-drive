import { Component, HostListener, Input } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIcon } from "@angular/material/icon";
import { MatToolbarModule } from "@angular/material/toolbar";
import { Router, ActivatedRoute } from "@angular/router";
import { MediaService } from "../media/media.service";

@Component({
  selector: "lg-navbar",
  standalone: true,
  imports: [MatToolbarModule, MatButtonModule, MatIcon],
  templateUrl: "./navbar.component.html",
  styles: `
  .mat-toolbar {
  color: white;
}
.mat-button {
  color: white;
}
.spacer {
  flex: 1 1 auto;
}
.hide-toolbar {
      transform: translateY(-100%);
      transition: transform 0.3s ease-in-out;
    }
  `,
})
export class NavbarComponent {
  @Input() title!: string;
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private ms: MediaService
  ) {}
  back() {
    const pathNow = this.route.snapshot.queryParams["path"];
    if (pathNow.includes("/")) {
      let parentPathArray = pathNow.split("/");
      parentPathArray.pop();
      let parentPath = parentPathArray.join("/");
      this.router.navigate(["/album"], {
        queryParams: { path: parentPath },
      });
    } else {
      // Use the navigate method with a relative path
      this.router.navigate(["../"], { relativeTo: this.route });
    }

    this.ms.clearStates();
  }

  hideToolbar = false;
  lastScrollTop = 0;

  @HostListener("window:scroll", ["$event"])
  onWindowScroll() {
    let currentScrollTop =
      window.pageYOffset || document.documentElement.scrollTop;
    if (currentScrollTop > this.lastScrollTop) {
      this.hideToolbar = true;
    } else {
      this.hideToolbar = false;
    }
    this.lastScrollTop = currentScrollTop <= 0 ? 0 : currentScrollTop; // For Mobile or negative scrolling
  }
}
