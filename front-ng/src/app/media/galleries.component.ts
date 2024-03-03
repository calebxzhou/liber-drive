import {Component, HostListener, OnInit} from "@angular/core";
import {MediaService} from "./media.service";
import {Gallery} from './media';
import {CommonModule} from "@angular/common";
import {toReadableSize} from "../util";
import {MatGridListModule} from '@angular/material/grid-list';
import { GalleryComponent } from "../gallery/gallery.component";

@Component({
    selector: "galleries",
    templateUrl: "./galleries.component.html",
    imports: [CommonModule, GalleryComponent],
    standalone: true,
})
export class GalleriesComponent implements OnInit {
    galleries: Gallery[] = [];

    constructor(private galleryService: MediaService) {
    }

    @HostListener('window:resize', ['$event'])
    onResize(event: Event) {
        if (event.target) {
            console.log((event.target as Window).innerWidth);
        }
    }

    ngOnInit() {
        this.getGalleries();
    }


    getGalleries(): void {
        this.galleryService
            .getGalleries()
            .subscribe(
                (galleries) =>

                    this.galleries = galleries
                        .filter(g => g.size > 0)
                        .sort((g1, g2) => g1.name.localeCompare(g2.name))
                        .reverse()
            );

    }
    getGalleryForAllMedias(): Gallery{
        let allGalleries: Gallery = {
            id: -1,
            name: "all galleries",
            size: 0,
            medias: new Map<number, MediaItem>()
          };
          
          // Loop through the Gallery array and accumulate the size and medias
          for (let gallery of Gallery[]) {
            // Add the size of each gallery to the total size
            allGalleries.size += gallery.size;
          
            // Loop through the medias map of each gallery
            for (let [key, value] of gallery.medias) {
              // Check if the key already exists in the aggregated medias map
              if (allGalleries.medias.has(key)) {
                // If yes, update the value with some logic (e.g. merge, overwrite, etc.)
                // This depends on how you want to handle duplicate keys
                let existingValue = allGalleries.medias.get(key);
                let newValue = mergeMediaItems(existingValue, value); // Define this function as per your logic
                allGalleries.medias.set(key, newValue);
              } else {
                // If no, simply add the key-value pair to the aggregated medias map
                allGalleries.medias.set(key, value);
              }
            }
          }

          return allGalleries;
    }
}
