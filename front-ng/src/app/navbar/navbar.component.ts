import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, NavigationEnd, Router, RouterModule } from '@angular/router';
import { PageService } from '../page.service';

@Component({
  selector: 'navbar',
  providers: [],
  standalone:true,
  templateUrl: './navbar.component.html',
})
export class NavbarComponent implements OnInit{

  title:string='';
	constructor(private page:PageService) {
    
	}
  ngOnInit(): void {
    this.page.path$.subscribe((newPath) => {
     this.title=this.page.getPathUrl().replaceAll("/",">");
     
  });
  }
  
  goBack(){
    this.page.goPrevPath();
  }

}
