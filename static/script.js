async function fetchData() {
    try {
        const response = await fetch('/api/data');
        const data = await response.json();
        updateUI(data);
    } catch (error) {
        console.error('Error fetching data:', error);
    }
}

function updateUI(data) {
    // Identity & Config
    document.getElementById('callsign').innerText = data.config.callsign;
    document.getElementById('grid-square').innerText = data.config.grid_square;
    document.getElementById('lat').innerText = data.config.latitude;
    document.getElementById('lon').innerText = data.config.longitude;

    // Weather
    if (data.weather) {
        document.getElementById('temp').innerText = data.weather.temperature;
        document.getElementById('hum').innerText = data.weather.humidity;
        document.getElementById('chill').innerText = data.weather.wind_chill;
    }

    // Solar
    if (data.solar) {
        document.getElementById('sfi').innerText = data.solar.sfi;
        document.getElementById('sn').innerText = data.solar.sn;
        document.getElementById('a-idx').innerText = data.solar.a_index;
        document.getElementById('k-idx').innerText = data.solar.k_index;
        document.getElementById('x-ray').innerText = data.solar.x_ray;

        const propList = document.getElementById('prop-list');
        propList.innerHTML = '';
        data.solar.band_conditions.forEach(([band, status]) => {
            const li = document.createElement('li');
            li.innerHTML = `${band}: <span class="${status.toLowerCase()}">${status}</span>`;
            propList.appendChild(li);
        });
    }

    // Satellites
    if (data.satellites) {
        const satList = document.getElementById('sat-list');
        satList.innerHTML = '';
        data.satellites.forEach(sat => {
            const li = document.createElement('li');
            const aosTime = new Date(sat.aos).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
            li.innerText = `${sat.name} @ ${aosTime} (Max: ${Math.round(sat.max_el)}°)`;
            satList.appendChild(li);
        });
    }

    // News
    if (data.news && data.news.headlines.length > 0) {
        const ticker = document.getElementById('ticker-content');
        const separator = ' <span class="ticker-separator">|</span> ';
        ticker.innerHTML = data.news.headlines.join(separator);
    }

    // Map
    drawMap();
}

function updateClock() {
    const now = new Date();
    const utcTime = now.toISOString().split('T')[1].split('.')[0];
    const utcDate = now.toISOString().split('T')[0];
    document.getElementById('clock').innerText = utcTime + " UTC";
    document.getElementById('date').innerText = utcDate;
}

// Map drawing logic (Simplified daylight check in JS)
function drawMap() {
    const canvas = document.getElementById('map-canvas');
    const ctx = canvas.getContext('2d');
    const width = canvas.width = canvas.parentElement.clientWidth;
    const height = canvas.height = canvas.parentElement.clientHeight;

    const now = new Date();
    const dayOfYear = Math.floor((now - new Date(now.getFullYear(), 0, 0)) / (1000 * 60 * 60 * 24));
    const hour = now.getUTCHours() + now.getUTCMinutes() / 60 + now.getUTCSeconds() / 3600;
    
    // Approximate solar declination
    const declination = 23.45 * Math.sin((2 * Math.PI * (284 + dayOfYear)) / 365);
    const declRad = declination * (Math.PI / 180);

    const imgData = ctx.createImageData(width, height);
    for (let y = 0; y < height; y++) {
        for (let x = 0; x < width; x++) {
            const lat = 90 - (y / height) * 180;
            const lon = (x / width) * 360 - 180;
            const latRad = lat * (Math.PI / 180);

            const solarTime = (hour + lon / 15 + 24) % 24;
            const hourAngle = (15 * (solarTime - 12)) * (Math.PI / 180);

            const altitude = Math.asin(
                Math.sin(latRad) * Math.sin(declRad) +
                Math.cos(latRad) * Math.cos(declRad) * Math.cos(hourAngle)
            );

            const i = (y * width + x) * 4;
            
            // Soft terminator transition
            const softTerminator = Math.max(0, Math.min(1, altitude * 10 + 0.5));
            const isNight = altitude <= 0;
            
            if (isNight) {
                // Night shadow (Dark Blue/Gray)
                imgData.data[i] = 20;
                imgData.data[i+1] = 20;
                imgData.data[i+2] = 40;
                imgData.data[i+3] = 180; // Semi-transparent
            } else {
                // Day (Transparent to show background map)
                imgData.data[i] = 0;
                imgData.data[i+1] = 0;
                imgData.data[i+2] = 0;
                imgData.data[i+3] = 0;
            }
        }
    }
    ctx.putImageData(imgData, 0, 0);
}

setInterval(fetchData, 5000);
setInterval(updateClock, 1000);
fetchData();
updateClock();
window.onresize = drawMap;
