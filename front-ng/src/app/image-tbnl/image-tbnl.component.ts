import { Component, OnInit, Input, OnDestroy } from '@angular/core';
import { CommonModule } from '@angular/common';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { HttpEvent, HttpEventType } from '@angular/common/http';
import { Subscription } from 'rxjs';
import { MediaService } from '../media/media.service';
import { trigger, state, style, animate, transition } from '@angular/animations';
@Component({
  selector: 'lg-image-tbnl',
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule],
  templateUrl: './image-tbnl.component.html',
  styles: `
  `,
    animations: [
      trigger('scaleAnimation', [
        state('initial', style({ transform: 'scale(0.01)' })),
        state('normal', style({ transform: 'scale(1)' })),
        transition('initial => normal', animate('500ms ease-in'))
      ])
    ]
})
// 缩略图
export class ImageTbnlComponent implements OnInit   {
  @Input() url!: string;
  @Input() isVideo: boolean = false; 
  state: string = 'initial';
  videoDuration = "视频"; 

  constructor(private ms: MediaService) {}

  ngOnInit(): void { 
    setTimeout(() => {
      this.state = 'normal';
    }, 0);
    if (this.isVideo) {
      this.fetchDuration();
    }
  }

  async fetchDuration() {
    let sec = await this.ms.getVideoDuration(this.url.replaceAll("?tbnl=1",""));
    this.videoDuration = this.ms.formatDuration(sec);
  }
 
}
