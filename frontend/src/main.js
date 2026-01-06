import './style.css';

let currentAvailabilities = {};
let customFilters = []; // { id, name, type, weekdays, startTime, endTime }
let currentFilterId = 'all';
let deleteMode = false;
let currentMonthDate = new Date(2026, 0, 1);

async function init() {
  const app = document.querySelector('#app');
  app.innerHTML = `
    <header>
      <h1>Réservation de Padel</h1>
    </header>

    <div class="filter-bar">
      <span class="filter-label">Filtres</span>
      <button id="btn-create-filter" class="tag-action-btn" title="Créer un filtre">+</button>
      <button id="btn-delete-mode" class="tag-action-btn" title="Supprimer un filtre">-</button>
      <div id="filter-container" class="filter-tags-list">
        <!-- Rendered tags will go here -->
      </div>
    </div>

    <div class="month-nav">
      <button id="prev-month" class="nav-btn" title="Mois précédent">&lt;</button>
      <div id="month-label" class="month-label"></div>
      <button id="next-month" class="nav-btn" title="Mois suivant">&gt;</button>
    </div>

    <div class="calendar-container">
      <div class="calendar-header">
        <div class="weekday">Lun</div>
        <div class="weekday">Mar</div>
        <div class="weekday">Mer</div>
        <div class="weekday">Jeu</div>
        <div class="weekday">Ven</div>
        <div class="weekday">Sam</div>
        <div class="weekday">Dim</div>
      </div>
      <div id="calendar-grid" class="calendar-grid"></div>
    </div>
    
    <!-- Availabilities Modal -->
    <div id="modal-overlay" class="modal-overlay">
      <div class="modal-content">
        <button id="close-modal" class="close-btn">&times;</button>
        <h2 id="modal-title" class="modal-title"></h2>
        <div id="modal-body"></div>
      </div>
    </div>

    <!-- Booking Modal -->
    <div id="booking-modal" class="modal-overlay">
      <div class="modal-content">
        <button id="close-booking" class="close-btn">&times;</button>
        <h2 id="booking-title" class="modal-title">Détails du créneau</h2>
        <div id="booking-body" class="booking-details"></div>
        <button id="reserve-btn" class="reserve-btn disabled" disabled>Réserver</button>
      </div>
    </div>

    <!-- Create Filter Modal -->
    <div id="create-filter-modal" class="modal-overlay">
      <div class="modal-content">
        <button id="close-create-filter" class="close-btn">&times;</button>
        <h2 class="modal-title">Créer un filtre</h2>
        <form id="filter-form">
          <div class="form-group">
            <label>Nom du filtre</label>
            <input type="text" id="filter-name" required placeholder="Ex: Soirée Semaine">
          </div>
          <div class="form-group">
            <label>Type de terrain</label>
            <div class="checkbox-group">
              <label class="checkbox-item">
                <input type="checkbox" id="type-all" checked> Tous
              </label>
              <label class="checkbox-item">
                <input type="checkbox" id="type-indoor" checked> Intérieur
              </label>
              <label class="checkbox-item">
                <input type="checkbox" id="type-outdoor" checked> Extérieur
              </label>
            </div>
          </div>
          <div class="form-group">
            <label>Jours de la semaine</label>
            <div class="weekday-selector">
              ${['Lun', 'Mar', 'Mer', 'Jeu', 'Ven', 'Sam', 'Dim'].map((day, i) => `
                <input type="checkbox" id="day-${i}" class="weekday-checkbox" checked>
                <label for="day-${i}" class="weekday-label">${day}</label>
              `).join('')}
            </div>
          </div>
          <div class="form-group">
            <label>Plage horaire</label>
            <div class="range-slider">
              <div class="progress" id="slider-progress"></div>
            </div>
            <div class="range-input">
              <input type="range" id="min-time" min="0" max="1439" value="480" step="30">
              <input type="range" id="max-time" min="0" max="1439" value="1320" step="30">
            </div>
            <div class="range-values">
              <span id="min-val">08:00</span>
              <span id="max-val">22:00</span>
            </div>
          </div>
          <button type="submit" class="reserve-btn">Enregistrer</button>
        </form>
      </div>
    </div>
  `;

  initSlider();
  setupFilterListeners();
  renderFilterTags();

  document.getElementById('prev-month').onclick = () => {
    currentMonthDate.setMonth(currentMonthDate.getMonth() - 1);
    renderCalendar(currentAvailabilities);
  };
  document.getElementById('next-month').onclick = () => {
    currentMonthDate.setMonth(currentMonthDate.getMonth() + 1);
    renderCalendar(currentAvailabilities);
  };

  const calendarGrid = document.querySelector('#calendar-grid');
  document.getElementById('close-modal').onclick = closeModal;
  document.getElementById('modal-overlay').onclick = (e) => {
    if (e.target.id === 'modal-overlay') closeModal();
  };

  document.getElementById('close-booking').onclick = closeBookingModal;
  document.getElementById('booking-modal').onclick = (e) => {
    if (e.target.id === 'booking-modal') closeBookingModal();
  };

  // Load mock data
  try {
    const response = await fetch('/mock_calendar.json');
    const data = await response.json();
    currentAvailabilities = data.availabilities;
    renderCalendar(currentAvailabilities);
  } catch (error) {
    console.error('Failed to load calendar data:', error);
    calendarGrid.innerHTML = '<div style="grid-column: span 7; padding: 20px; text-align: center;">Échec du chargement des données</div>';
  }
}

function initSlider() {
  const minInput = document.getElementById('min-time');
  const maxInput = document.getElementById('max-time');
  const minVal = document.getElementById('min-val');
  const maxVal = document.getElementById('max-val');
  const progress = document.getElementById('slider-progress');

  const update = () => {
    let min = parseInt(minInput.value);
    let max = parseInt(maxInput.value);

    // Ensure handles don't cross (min gap 60 min)
    if (max - min < 60) {
      if (minInput === document.activeElement) {
        minInput.value = max - 60;
        min = max - 60;
      } else {
        maxInput.value = min + 60;
        max = min + 60;
      }
    }

    const minHrs = Math.floor(min / 60);
    const minMins = min % 60;
    const maxHrs = Math.floor(max / 60);
    const maxMins = max % 60;

    minVal.textContent = `${String(minHrs).padStart(2, '0')}:${String(minMins).padStart(2, '0')}`;
    maxVal.textContent = `${String(maxHrs).padStart(2, '0')}:${String(maxMins).padStart(2, '0')}`;

    progress.style.left = (min / 1440) * 100 + '%';
    progress.style.right = 100 - (max / 1440) * 100 + '%';
  };

  minInput.oninput = update;
  maxInput.oninput = update;
  update();
}

function setupFilterListeners() {
  const typeAll = document.getElementById('type-all');
  const typeIndoor = document.getElementById('type-indoor');
  const typeOutdoor = document.getElementById('type-outdoor');

  typeAll.onchange = () => {
    typeIndoor.checked = typeAll.checked;
    typeOutdoor.checked = typeAll.checked;
  };

  [typeIndoor, typeOutdoor].forEach(cb => {
    cb.onchange = () => {
      if (!cb.checked) typeAll.checked = false;
      if (typeIndoor.checked && typeOutdoor.checked) typeAll.checked = true;
    };
  });

  document.getElementById('btn-create-filter').onclick = () => {
    document.getElementById('create-filter-modal').classList.add('active');
  };

  document.getElementById('close-create-filter').onclick = () => {
    document.getElementById('create-filter-modal').classList.remove('active');
  };

  document.getElementById('btn-delete-mode').onclick = (e) => {
    deleteMode = !deleteMode;
    e.currentTarget.classList.toggle('active', deleteMode);
    renderFilterTags();
  };

  document.getElementById('filter-form').onsubmit = (e) => {
    e.preventDefault();
    const weekdays = [];
    for (let i = 0; i < 7; i++) {
      if (document.getElementById(`day-${i}`).checked) weekdays.push(i);
    }

    const newFilter = {
      id: 'filter-' + Date.now().toString(),
      name: document.getElementById('filter-name').value,
      types: {
        indoor: typeIndoor.checked,
        outdoor: typeOutdoor.checked
      },
      weekdays: weekdays,
      startTime: document.getElementById('min-val').textContent,
      endTime: document.getElementById('max-val').textContent
    };

    customFilters.push(newFilter);
    document.getElementById('create-filter-modal').classList.remove('active');
    document.getElementById('filter-form').reset();
    typeAll.checked = true;
    typeIndoor.checked = true;
    typeOutdoor.checked = true;
    renderFilterTags();
    renderCalendar(currentAvailabilities);
  };

  // Delegate filter clicks
  document.getElementById('filter-container').onclick = (e) => {
    const btn = e.target.closest('.filter-btn, .filter-tag');
    if (!btn) return;

    const id = btn.dataset.id;
    if (deleteMode && id !== 'all') {
      customFilters = customFilters.filter(f => f.id !== id);
      if (currentFilterId === id) currentFilterId = 'all';
      renderFilterTags();
      renderCalendar(currentAvailabilities);
    } else {
      currentFilterId = id;
      renderFilterTags();
      renderCalendar(currentAvailabilities);
    }
  };
}

function renderFilterTags() {
  const container = document.getElementById('filter-container');
  const defaults = `
    <button data-id="all" class="filter-btn ${currentFilterId === 'all' ? 'active' : ''}">Tous</button>
  `;

  const custom = customFilters.map(f => `
    <button data-id="${f.id}" class="filter-tag ${currentFilterId === f.id ? 'active' : ''} ${deleteMode ? 'to-delete' : ''}">
      ${deleteMode ? '× ' : ''}${f.name}
    </button>
  `).join('');

  container.innerHTML = defaults + custom;
}

function matchesFilter(slot, playground, dateStr) {
  // Get active filter
  let filter;
  if (currentFilterId === 'all') {
    filter = { types: { indoor: true, outdoor: true }, weekdays: [0, 1, 2, 3, 4, 5, 6], startTime: '00:00', endTime: '23:59' };
  } else {
    filter = customFilters.find(f => f.id === currentFilterId);
  }

  if (!filter) return false;

  // 1. Type check
  if (playground.indoor && !filter.types.indoor) return false;
  if (!playground.indoor && !filter.types.outdoor) return false;

  // 2. Weekday check
  const date = new Date(dateStr);
  let dayOfWeek = date.getDay() - 1;
  if (dayOfWeek === -1) dayOfWeek = 6;
  if (!filter.weekdays.includes(dayOfWeek)) return false;

  // 3. Time range check
  if (slot.startAt < filter.startTime || slot.startAt > filter.endTime) return false;

  // 4. Duration check
  return slot.prices.some(price => price.bookable && price.duration >= 5400);
}

function renderCalendar(availabilities) {
  const grid = document.querySelector('#calendar-grid');
  const monthLabel = document.querySelector('#month-label');

  const year = currentMonthDate.getFullYear();
  const month = currentMonthDate.getMonth();

  const date = new Date(year, month, 1);
  // firstDay index: 0=Sun, 1=Mon, ..., 6=Sat. 
  // We want 0=Mon, 1=Tue, ..., 6=Sun
  let firstDay = date.getDay() - 1;
  if (firstDay === -1) firstDay = 6; // Sunday becomes 6

  const daysInMonth = new Date(year, month + 1, 0).getDate();

  monthLabel.textContent = date.toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' });

  grid.innerHTML = '';

  // Padding mois précédent
  const prevMonthLastDay = new Date(year, month, 0).getDate();
  for (let i = 0; i < firstDay; i++) {
    const dayElement = createDayElement(prevMonthLastDay - (firstDay - 1 - i), true);
    grid.appendChild(dayElement);
  }

  // Jours du mois en cours
  for (let d = 1; d <= daysInMonth; d++) {
    const dateStr = `${year}-${String(month + 1).padStart(2, '0')}-${String(d).padStart(2, '0')}`;
    const dayAvail = availabilities[dateStr];

    let hasAvailability = false;
    if (dayAvail && dayAvail['hydra:member']) {
      hasAvailability = dayAvail['hydra:member'].some(playground =>
        playground.activities.some(activity =>
          activity.slots.some(slot => matchesFilter(slot, playground, dateStr))
        )
      );
    }

    const dayElement = createDayElement(d, false, hasAvailability, dateStr);
    grid.appendChild(dayElement);
  }
}

function createDayElement(day, isOtherMonth, isAvailable = false, dateStr = '') {
  const div = document.createElement('div');
  div.className = `calendar-day ${isOtherMonth ? 'other-month' : ''}`;

  if (!isOtherMonth) {
    div.classList.add(isAvailable ? 'status-available' : 'status-unavailable');
    if (isAvailable) {
      div.style.cursor = 'pointer';
      div.onclick = () => showModal(dateStr);
    }
  }

  const today = new Date();
  if (!isOtherMonth && day === today.getDate() && 2026 === today.getFullYear() && 0 === today.getMonth()) {
    div.classList.add('today');
  }

  div.innerHTML = `<div class="day-number">${day}</div>`;

  return div;
}

function showModal(dateStr) {
  const overlay = document.getElementById('modal-overlay');
  const title = document.getElementById('modal-title');
  const body = document.getElementById('modal-body');

  const formattedDate = new Date(dateStr).toLocaleDateString('fr-FR', {
    weekday: 'long', month: 'long', day: 'numeric'
  });

  title.textContent = `Disponibilités pour le ${formattedDate}`;
  body.innerHTML = '';

  const dayAvail = currentAvailabilities[dateStr];
  if (dayAvail && dayAvail['hydra:member']) {
    dayAvail['hydra:member'].forEach(playground => {
      // Group durations by startAt for the same playground
      const slotsByTime = {};

      playground.activities.forEach(activity => {
        activity.slots.forEach(slot => {
          if (matchesFilter(slot, playground, dateStr)) {
            if (!slotsByTime[slot.startAt]) {
              slotsByTime[slot.startAt] = {
                startAt: slot.startAt,
                durations: new Set(),
                courtName: playground.name
              };
            }
            slot.prices.forEach(price => {
              if (price.bookable && price.duration >= 5400) {
                slotsByTime[slot.startAt].durations.add(price.duration);
              }
            });
          }
        });
      });

      const timePoints = Object.keys(slotsByTime).sort();

      if (timePoints.length > 0) {
        const courtDiv = document.createElement('div');
        courtDiv.className = 'court-item';

        const courtName = document.createElement('div');
        courtName.className = 'court-name';
        courtName.textContent = playground.name;
        courtDiv.appendChild(courtName);

        const slotList = document.createElement('div');
        slotList.className = 'slot-list';

        timePoints.forEach(time => {
          const slotGroup = slotsByTime[time];
          const span = document.createElement('span');
          span.className = 'slot-tag bookable';
          span.textContent = time;

          span.addEventListener('click', (e) => {
            e.stopPropagation();
            showBookingModal(slotGroup);
          });

          slotList.appendChild(span);
        });

        courtDiv.appendChild(slotList);
        body.appendChild(courtDiv);
      }
    });
  }

  overlay.classList.add('active');
}

function showBookingModal(slotGroup) {
  const overlay = document.getElementById('booking-modal');
  const body = document.getElementById('booking-body');

  // Sort and format durations
  const sortedDurations = Array.from(slotGroup.durations).sort();
  const durationText = sortedDurations.map(d => {
    const mins = d / 60;
    return mins === 90 ? '1h30' : '2h';
  }).join(' ou ');

  body.innerHTML = `
    <p><strong>Terrain :</strong> ${slotGroup.courtName}</p>
    <p><strong>Heure :</strong> ${slotGroup.startAt}</p>
    <p><strong>Durées disponibles :</strong> ${durationText}</p>
  `;

  overlay.classList.add('active');
}

function closeBookingModal() {
  document.getElementById('booking-modal').classList.remove('active');
}

function closeModal() {
  document.getElementById('modal-overlay').classList.remove('active');
}

init();
