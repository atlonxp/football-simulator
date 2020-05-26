import { Injectable } from "@angular/core";
import { Observable, Subject } from "rxjs";

@Injectable({
    providedIn: 'root',
})
export class HeaderService {
    public titleSubject: Subject<HeaderModel> = new Subject<HeaderModel>();

    setHeader(title: String, subTitle: String){
        this.titleSubject.next(new HeaderModel(title, subTitle));
    }

    get(): Observable<HeaderModel> {
        return this.titleSubject.asObservable();
    }
}

export class HeaderModel {
    constructor(title: String, subTitle: String){
        this.title = title;
        this.subTitle = subTitle;
    }

    public title: String;
    public subTitle: String;
}