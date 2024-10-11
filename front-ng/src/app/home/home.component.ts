import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { Router, RouterModule } from "@angular/router";
import { Title } from "@angular/platform-browser";
import { MatButtonModule } from "@angular/material/button";
import { MediaService } from "../media/media.service";
import { MatGridListModule } from "@angular/material/grid-list";
import { MatListModule } from "@angular/material/list";
import { Album, AlbumInfo, Media } from "../media/media";
import { toReadableSize } from "../util";
import { Pipe, PipeTransform } from "@angular/core";
import { AlbumTbnlComponent } from "../album-tbnl/album-tbnl.component";

@Component({
  selector: "lg-home",
  standalone: true,
  imports: [
    CommonModule,
    RouterModule,
    MatButtonModule,
    MatGridListModule,
    MatListModule,
    AlbumTbnlComponent,
  ],
  templateUrl: "./home.component.html",
  styles: `
  
  .grid-container {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(128px, 0.8fr));
    grid-auto-rows: 1fr;
    grid-gap: 1px;
  }
 
  `,
})
export class HomeComponent implements OnInit {
  albums: AlbumInfo[] = [];
  constructor(private router: Router, private ms: MediaService) {}
  ngOnInit(): void {
    this.ms.pwd = undefined;
    this.ms.listAllAlbums().subscribe((albums) => {
      this.albums = Object.keys(albums)
        .map((albumName) => {
          let mediaNames = albums[albumName];
          return {
            name: albumName,
            tbnl_ids: mediaNames,
          };
        })
        .sort((a, b) => a.name.localeCompare(b.name)); // Sort alphabetically by album name
    });
  }
}
