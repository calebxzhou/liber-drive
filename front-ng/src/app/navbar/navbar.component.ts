import { Component, Input } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatIcon } from "@angular/material/icon";
import { MatToolbarModule } from "@angular/material/toolbar";
import { Router, ActivatedRoute } from "@angular/router";

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

  `,
})
export class NavbarComponent {
  @Input() title!:string;
  constructor(private router: Router, private route: ActivatedRoute) {}
  back() {
    // Use the navigate method with a relative path
    this.router.navigate(["../"], { relativeTo: this.route });
  }
}
