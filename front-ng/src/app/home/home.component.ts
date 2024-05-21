import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { Router, RouterModule } from "@angular/router";
import { AlbumPreviewComponent } from "../album-preview/album-preview.component";
import { Title } from "@angular/platform-browser";
import { MatButtonModule } from "@angular/material/button";

@Component({
  selector: "lg-home",
  standalone: true,
  imports: [CommonModule, AlbumPreviewComponent, RouterModule, MatButtonModule],
  templateUrl: "./home.component.html",
  styles: ``,
})
export class HomeComponent implements OnInit {
  constructor(private router: Router, private titleService: Title) {}
  ngOnInit(): void {}
  goGallery(name: string) {
    this.router.navigate(["/gallery/" + name]);
  }
}
