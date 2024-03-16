import { Component, OnInit } from "@angular/core";
import { GalleryInfo, DefaultGallery } from "../media/media";
import { ActivatedRoute } from "@angular/router";
import { MediaService } from "../media/media.service";
import { NavbarComponent } from "../navbar/navbar.component";
import { toReadableSize } from "../util";
import { AlbumGridComponent } from "../album-grid/album-grid.component";

@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [NavbarComponent, AlbumGridComponent],
  templateUrl: "./gallery.component.html",
  styles: ``,
})
export class GalleryComponent implements OnInit {
  gallery$: GalleryInfo = DefaultGallery;
  constructor(private route: ActivatedRoute, private mserv: MediaService) {}
  ngOnInit() {
    this.route.paramMap.subscribe((params) => {
      this.mserv.fetchGallery(params.get("galleryName")!).subscribe((g) => {
        g.albums = g.albums.sort();
        this.gallery$ = g;
      });
    });
  }
  getTitle() {
    return `「${this.gallery$.name}」${
      this.gallery$.albums.length
    }个影集 ${toReadableSize(this.gallery$.size)}`;
  }
}
