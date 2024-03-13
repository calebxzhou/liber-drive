import { Component, OnInit, Input, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HttpEvent, HttpEventType } from '@angular/common/http';
import { Subscription } from 'rxjs';
import { MediaService } from '../media/media.service';

@Component({
  selector: 'lg-image-tbnl',
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule],
  templateUrl: './image-tbnl.component.html',
  styles: ``
})
// 缩略图
export class ImageTbnlComponent implements OnInit, OnDestroy {
  @Input() url!: string;
  @Input() isVideo: boolean = false;
  @Input() sidelen!: number;
  videoDuration = "";
  progress: number = 0;
  imageData: string | null = null;
  private fetchSubscription: Subscription | null = null;

  constructor(private ms: MediaService) {}

  ngOnInit(): void {
    this.fetchSubscription = this.ms.fetchBlob(this.url).subscribe((event: HttpEvent<any>) => {
      if (event.type === HttpEventType.DownloadProgress) {
        this.progress = Math.round((100 * event.loaded) / (event.total ?? 1));
      } else if (event.type === HttpEventType.Response) {
        const blob: Blob = event.body;
        this.imageData = URL.createObjectURL(blob);
      }
    });

    if (this.isVideo) {
      this.fetchDuration();
    }
  }

  async fetchDuration() {
    let sec = await this.ms.getVideoDuration(this.url.replaceAll("?tbnl=1",""));
    this.videoDuration = this.ms.formatDuration(sec);
  }

  ngOnDestroy(): void {
    if (this.fetchSubscription) {
      this.fetchSubscription.unsubscribe();
    }
  }
}
