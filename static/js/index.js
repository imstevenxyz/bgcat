"use strict";

let pagination;
let liveSearchTimer;
let boardgameContainer = document.getElementById("boardgame-container");
let boardgameQueryForm = document.getElementById("query");
let boardgameSearchBar = document.getElementById("search");
let boardgame_query = "";

(function () {
  const currentPage = Number(document.getElementById("pagination-bgs").getAttribute("initial-page"));
  const totalPages = Number(document.getElementById("pagination-bgs").getAttribute("page-count"));
  pagination = new Pagination(currentPage, totalPages, fetch_boardgames);
  boardgame_query = generate_boardgame_query();
  setup_boardgame_query_form();
  setup_live_search();
})();

/*
 * Setup live search bar
 */
function setup_live_search() {
  boardgameSearchBar.addEventListener("keyup", (event) => {
    if (event.key !== 'Enter') {
      clearTimeout(liveSearchTimer);
      liveSearchTimer = setTimeout(function(){
          filter_boardgames();
      }, 500);
    }else{
      clearTimeout(liveSearchTimer);
    }
  });
}

/*
 * Setup query form
 */
function setup_boardgame_query_form() {
  boardgameSearchBar.setAttribute("form", "query");
  boardgameQueryForm.addEventListener("submit", (submit) => {
    submit.preventDefault();
    filter_boardgames();
  });
}

function filter_boardgames(){
  pagination.clear();
  pagination.show_loader();
  boardgame_query = generate_boardgame_query();
  fetch_boardgames(0).then((pageCount) => {
    pagination.hide_loader();
    pagination.update_context(0, pageCount);
  });
}

/*
 * Generate query parameters from the query form
 */
function generate_boardgame_query() {
  let form = new FormData(boardgameQueryForm);
  for (let [name, value] of Array.from(form.entries())) {
    if (value === '') form.delete(name);
  }
  let queryString = new URLSearchParams(form).toString();
  return "&" + queryString;
}

/*
 * Fetch boardgames for a specific page
 */
async function fetch_boardgames(pageIndex) {
  return fetch(`/api/v1/boardgames?page=${pageIndex}` + boardgame_query)
    .then(async (response) => {
      const boardgames = await response.json();
      boardgames.forEach((boardgame) => {
        add_boardgame(boardgame, pageIndex);
      });
      return response.headers.get("pagination-count");
    });
}

/*
 * Add boardgame to the item list
 */
function add_boardgame(boardgame, pageIndex) {
  let players_html = `${boardgame.min_players}`;
  if (boardgame.min_players != boardgame.max_players) {
    players_html = players_html.concat(` &mdash; ${boardgame.max_players}`);
  }
  if (boardgame.players_no_limit) {
    players_html = players_html.concat(`+`);
  }

  let playtime_html = `${boardgame.min_playtime}`;
  if (boardgame.min_playtime != boardgame.max_playtime) {
    playtime_html = playtime_html.concat(` &mdash; ${boardgame.max_playtime}`);
  }
  if (boardgame.playtime_no_limit) {
    playtime_html = playtime_html.concat(`+`);
  }

  boardgameContainer.insertAdjacentHTML("beforeend",`
  <article class="boardgame pagination-item hidden" pagination-index="${pageIndex}">
    <a href="/boardgame/${boardgame.uid}">
      <h4>${boardgame.title}</h4>
      <img src="${boardgame.image_url}", alt="${boardgame.title}"/>
      <ul class="info">
        <li>
          <i class="fas fa-users"></i>
          ${players_html}
        </li>
        <li>
          <i class="far fa-clock"></i>
          ${playtime_html} min
        </li>
      </ul>
    </a>
  </article>
  `)
}