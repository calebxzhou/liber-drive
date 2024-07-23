import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { Router, RouterModule } from "@angular/router";
import { AlbumPreviewComponent } from "../album-preview/album-preview.component";
import { Title } from "@angular/platform-browser";
import { MatButtonModule } from "@angular/material/button";
import { MediaService } from "../media/media.service";
import { MatGridListModule } from "@angular/material/grid-list";
import { MatListModule } from "@angular/material/list";
import { Album, AlbumInfo, Media } from "../media/media";
import { toReadableSize } from "../util";
import { Pipe, PipeTransform } from "@angular/core";
import { AlbumGridComponent } from "../album-grid/album-grid.component";

@Component({
  selector: "lg-home",
  standalone: true,
  imports: [
    CommonModule,
    AlbumPreviewComponent,
    RouterModule,
    MatButtonModule,
    MatGridListModule,
    MatListModule,
    AlbumGridComponent,
  ],
  templateUrl: "./home.component.html",
  styles: ``,
})
export class HomeComponent implements OnInit {
  albums: AlbumInfo[] = [];
  constructor(private router: Router, private ms: MediaService) {}
  ngOnInit(): void {
    //修改时间倒序
    this.ms.fetchAlbumList().subscribe((albums) => {
      this.albums = Object.keys(albums)
        .map((albumName) => {
          let media = albums[albumName];
          return {
            name: albumName,
            tbnl_url: this.ms.getAlbumTbnlUrl(albumName, media),
            latest_media_time: media.time,
          };
        })
        .sort((a, b) => b.latest_media_time - a.latest_media_time);
    });
  }
  goAlbum(name: string) {
    this.router.navigate(["/" + name]);
  }
}
