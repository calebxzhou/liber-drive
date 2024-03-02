import { Component, OnInit } from "@angular/core";
import { GalleryService } from "./gallery.service";
import { Gallery } from './gallery';
import { CommonModule } from "@angular/common";
import { toReadableSize } from "../util";

@Component({
  selector: "gallery",
  templateUrl: "./gallery.component.html",
  imports: [CommonModule],
  standalone: true,
})
export class GalleryComponent implements OnInit {
  galleries: Gallery[] = [];

  constructor(private galleryService: GalleryService) {}

  ngOnInit() {
    this.getGalleries();
  }
  getSize(gallery:Gallery):string{
    return toReadableSize(gallery.size);
  }
  getGalleries(): void {
    this.galleryService
      .getGalleries()
      .subscribe(
        (galleries) =>
          
            this.galleries = galleries
            .filter(g => g.size>0)
            .sort((g1, g2) => g1.name.localeCompare(g2.name))
            .reverse()
            
      );

  }
}
