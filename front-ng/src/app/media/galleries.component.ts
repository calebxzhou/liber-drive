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

    getSize(gallery: Gallery): string {
        return toReadableSize(gallery.size);
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
}
