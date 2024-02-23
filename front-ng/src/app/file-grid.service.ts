import { HttpClient } from "@angular/common/http";
import { Observable } from "rxjs";
import { map } from "rxjs/operators";
import { FileItem } from "./file-item";
import { Injectable } from "@angular/core";
import { PageService } from "./page.service";

@Injectable({
  providedIn: "root",
})
export class FileGridService {
  constructor(private http: HttpClient, private page: PageService) {}

  fetchFileItems(path: string): Observable<FileItem[]> {
    const url = `http://${this.page.getHostName()}:7789/${path}`;

    return this.http
      .get<string[]>(url)
      .pipe(
        map((files) =>
          files.map((file) => new FileItem(file, `${url}/${file}`))
        )
      );
  }
}
