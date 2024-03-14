import { Component, HostListener, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { CommonModule } from "@angular/common";
import { AlbumPreviewComponent } from "../album-preview/album-preview.component";
import { ActivatedRoute, ParamMap, RouterModule } from "@angular/router";
import { AlbumInfo } from "../media/media";

@Component({
  selector: "lg-album-grid",
  templateUrl: "./album-grid.component.html",
  imports: [CommonModule, AlbumPreviewComponent, RouterModule],
  styles: ` .grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(128px, 0.8fr));
    grid-auto-rows: 1fr;
    grid-gap: 1px;
  }`,
  standalone: true,
})
export class AlbumGridComponent {
  @Input() albums!: AlbumInfo[];
}
