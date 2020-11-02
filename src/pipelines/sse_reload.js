const evtSource = new EventSource("/build_events");

evtSource.onmessage = ev => {
    console.log(ev);
};