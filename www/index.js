import {App} from 'meetup-obs-overlay';
import data from './data.json';
import './main.css';

// initialize app, pass in data from json object
const app = App.new()
    .with_title(data.meetup_title)
    .with_speaker(data.speaker);
// Set the title
app.set_title();
// set the speaker name
app.set_speaker();
// every 5 minutes display the next info message
// app.add_class("popup", "#other");
// setTimeout(() => {
//     app.remove_class("popup", "#other");
// }, 15000);
// document.querySelector('#other').classList.add('popup');