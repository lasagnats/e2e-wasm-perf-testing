import { Component } from '@angular/core';
import { Entry } from './models/entry';
import { HttpClient } from '@angular/common/http';
import { catchError, throwError } from 'rxjs';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrl: './app.component.scss'
})
export class AppComponent {
  title = 'performance-test';
  entryCount = 0;
  entries: Entry[] = [];

  constructor(private http: HttpClient){}

  generateData() {
      const url = `http://localhost:3000/data/${this.entryCount}`;

      this.http.get(url)
        .pipe(
          catchError(error => {
            return throwError(error);
          })
        )
        .subscribe(response => {
            this.entries = response as Entry[];
        });
  }

  clear() {
    this.entries = [];
  }
}
