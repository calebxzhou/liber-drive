import {Component, HostListener, OnInit} from "@angular/core";
import {MediaService} from "./media.service";
import {Gallery, MediaItem} from './media';
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
                (galleries) =>{
                    this.galleries = galleries
                        .filter(g => g.size > 0)
                        .sort((g1, g2) => g1.name.localeCompare(g2.name))
                        .reverse();
                      this.galleries = [this.getGalleryForAllMedias()].concat(this.galleries);
                      }


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
          for (let gallery of this.galleries) {
            // Add the size of each gallery to the total size
            allGalleries.size += gallery.size;
          
            // Loop through the medias map of each gallery
            for (let [key, value] of gallery.medias) {
             
                allGalleries.medias.set(key, value);
              
            }
          }

          return allGalleries;
    }
}
