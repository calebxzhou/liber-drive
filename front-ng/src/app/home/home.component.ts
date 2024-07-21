import { CommonModule } from "@angular/common";
import { Component, OnInit } from "@angular/core";
import { Router, RouterModule } from "@angular/router";
import { AlbumPreviewComponent } from "../album-preview/album-preview.component";
import { Title } from "@angular/platform-browser";
import { MatButtonModule } from "@angular/material/button";
import { MediaService } from "../media/media.service";
import { MatGridListModule } from "@angular/material/grid-list";
import { MatListModule } from "@angular/material/list";
import { Album, AlbumInfo } from "../media/media";
import { toReadableSize } from "../util";
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
  ],
  templateUrl: "./home.component.html",
  styles: ``,
})
export class HomeComponent implements OnInit {
  albumInfos: AlbumInfo[] = [];
  constructor(private router: Router, private ms: MediaService) {}
  ngOnInit(): void {
    //修改时间倒序
    this.ms.fetchAlbumList().subscribe((albums) => {
      this.albumInfos = Object.values(albums)
        .map((album) => {
          const mediaAmount = Object.keys(album.medias).length;
          const albumSize = Object.values(album.medias).reduce(
            (total, media) => total + media.size,
            0
          );
          const latestMediaTime = Math.max(
            ...Object.values(album.medias).map((media) => media.time)
          );
          return {
            name: album.name,
            size: toReadableSize(albumSize),
            media_amount: mediaAmount,
            tbnl_url: this.ms.getAlbumTbnlUrl(album),
            latest_media_time: latestMediaTime,
          };
        })
        .sort((a, b) => b.latest_media_time - a.latest_media_time);
    });
  }
  goAlbum(name: string) {
    this.router.navigate(["/" + name]);
  }
}
