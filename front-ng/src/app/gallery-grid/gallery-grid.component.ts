import { Component, HostListener, OnInit } from "@angular/core";
import { MediaService } from "./media.service";
import { Gallery, MediaItem } from "./media";
import { CommonModule } from "@angular/common";
import { toReadableSize } from "../util";
import { MatGridListModule } from "@angular/material/grid-list";
import { GalleryComponent } from "../gallery/gallery.component";

@Component({
  selector: "galleries",
  templateUrl: "./galleries.component.html",
  imports: [CommonModule, GalleryComponent],
  standalone: true,
})
export class GalleriesComponent implements OnInit {
  galleries: Gallery[] = [];

  constructor(private galleryService: MediaService) {}

  @HostListener("window:resize", ["$event"])
  onResize(event: Event) {
    if (event.target) {
      console.log((event.target as Window).innerWidth);
    }
  }

  ngOnInit() {
    this.getGalleries();
  }

  getGalleries(): void {
    this.galleryService.fetchAllGalleries().subscribe((g) => {
      let gAll = g
        .filter((g) => g.size > 0)
        .sort((g1, g2) => g1.name.localeCompare(g2.name))
        .reverse();
      let gAllPhotos: Gallery = {
        id: -1,
        name: "全部照片",
        size: 0,
        medias: [],
      };

      // Loop through the Gallery array and accumulate the size and medias
      gAll.forEach((gallery) => {
        // Add the size of each gallery to the total size
        gAllPhotos.size += gallery.size;

        // Loop through the medias map of each gallery
        for (let media of gallery.medias) {
          gAllPhotos.medias.push(media);
        }
      });
      this.galleries = [gAllPhotos].concat(gAll);
    });
  }
}
