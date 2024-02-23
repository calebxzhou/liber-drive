import { Injectable, InjectionToken } from "@angular/core";
import { FormControl } from "@angular/forms";
import { Inject } from "@angular/core";
import { BehaviorSubject, Observable } from "rxjs";

export const WINDOW = new InjectionToken<Window>("window");
@Injectable({
  providedIn: "root",
})
export class PageService {
  private pathSubject: BehaviorSubject<string[]> = new BehaviorSubject<
    string[]
  >([]);

  // Expose the observable to components
  path$: Observable<string[]> = this.pathSubject.asObservable();

  constructor(@Inject(WINDOW) private window: Window) {}

  //获取域名
  getHostName(): string {
    return this.window.location.hostname;
  }
  getPathUrl() {
    return this.pathSubject.getValue().join("/").replaceAll("//","/");
  }
  // Function to remove the last element from pathSubject
  goPrevPath() {
    const currentPath = this.pathSubject.getValue();
    if (currentPath.length > 0) {
      const updatedPath = currentPath.slice(0, -1); // Remove the last element
      this.pathSubject.next(updatedPath);
    }
  }
  goNextPath(dir:string){
    const v = this.pathSubject.getValue();
    v.push(dir);
    this.pathSubject.next(v);
  }
}
