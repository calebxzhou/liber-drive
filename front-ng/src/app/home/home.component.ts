import { CommonModule } from '@angular/common';
import { Component, OnInit } from '@angular/core';
import { Router, RouterModule } from '@angular/router';
import { GalleryItemComponent } from '../gallery-item/gallery-item.component';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'lg-home',
  standalone: true,
  imports: [CommonModule, GalleryItemComponent,RouterModule],
  templateUrl: './home.component.html',
  styles: ``
})
export class HomeComponent implements OnInit {
  constructor(private router: Router,private titleService: Title) {}
  ngOnInit(): void {
    this.titleService.setTitle("嘉乐周的光影世界 2.1")
  }
  goGallery(name:string){
    this.router.navigate(['/gallery/'+name])
  }
}
