import { Component } from "@angular/core";
import { MatButtonModule } from "@angular/material/button";
import { MatToolbarModule } from "@angular/material/toolbar";

@Component({
  selector: "lg-navbar",
  standalone: true,
  imports: [MatToolbarModule, MatButtonModule],
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
export class NavbarComponent {}
