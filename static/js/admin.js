"use strict";

let pagination;
let liveSearchTimer;
let boardgameContainer = document.getElementById("boardgame-list");
let boardgameSearchBar = document.getElementById("search");

const itemLimit = Number(document.getElementById("pagination-bgs").getAttribute("limit"));

(function () {
  const currentPage = Number(document.getElementById("pagination-bgs").getAttribute("initial-page"));
  const totalPages = Number(document.getElementById("pagination-bgs").getAttribute("page-count"));
  pagination = new Pagination(currentPage, totalPages, fetch_boardgames);
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
 * Filter and update boardgames
 */
function filter_boardgames(){
  pagination.clear();
  pagination.show_loader();
  fetch_boardgames(0).then((pageCount) => {
    pagination.hide_loader();
    pagination.update_context(0, pageCount);
  });
}

/*
 * Fetch boardgames for a specific page
 */
async function fetch_boardgames(pageIndex) {
 return fetch(`/api/v1/boardgames?limit=${itemLimit}&page=${pageIndex}&search=${boardgameSearchBar.value}`)
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
  boardgameContainer.insertAdjacentHTML("beforeend",`
  <li class="boardgame flex-bar pagination-item hidden" pagination-index="${pageIndex}">
      <div class="flex-start">
          <h4>${boardgame.title}</h4>
          <p id="uid" onclick="copyUid(this)">
          ${boardgame.uid}<span class="tooltip">Copied!</span>
          </p>
      </div>
      <div class="flex-end">
        <form action="/admin/edit/${boardgame.uid}" method="get">
          <button type="submit" id="edit">
            <i class="fas fa-edit fa-fw"></i> edit
          </button>
        </form>
        <form action="/admin/delete/${boardgame.uid}" method="post">
          <button type="submit" id="delete">
            <i class="fas fa-trash fa-fw"></i> delete
          </button>
        </form>
      </div>
  </li>
  `)
}

/*
 * Copy selected uid and show tooltip
 */
function copyUid(element){
  navigator.clipboard.writeText(element.innerText).then(function() {
    element.firstElementChild.classList.toggle("show");
    setTimeout(() => {
        element.firstElementChild.classList.toggle("show");
    }, 1000);
  }, function(err) {
    console.error('BGCat: Could not copy text ', err);
  });
}