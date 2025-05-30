"use strict";

let pagination;
let liveSearchTimer;
let boardgameContainer = document.getElementById("boardgame-list");
let boardgameSearchBar = document.getElementById("search");

const itemLimit = Number(
  document.getElementById("pagination-bgs").getAttribute("limit"),
);

(function () {
  const currentPage = Number(
    document.getElementById("pagination-bgs").getAttribute("initial-page"),
  );
  const totalPages = Number(
    document.getElementById("pagination-bgs").getAttribute("page-count"),
  );
  pagination = new Pagination(currentPage, totalPages, fetchBoardgames);
  modifyAvailabilityButtons();
  setupLiveSearch();
})();

/*
 * Modify boardgame availability buttons
 */
function modifyAvailabilityButtons() {
  const buttons_availability = document.querySelectorAll(
    ".boardgame button.availability",
  );
  buttons_availability.forEach((button) => {
    button.type = "button";
    button.addEventListener("click", () => {
      setBoardgameAvailability(button);
    });
  });
}

/*
 * Setup live search bar
 */
function setupLiveSearch() {
  boardgameSearchBar.addEventListener("keyup", (event) => {
    if (event.key !== "Enter") {
      clearTimeout(liveSearchTimer);
      liveSearchTimer = setTimeout(function () {
        filterBoardgames();
      }, 500);
    } else {
      clearTimeout(liveSearchTimer);
    }
  });
}

/*
 * Filter and update boardgames
 */
function filterBoardgames() {
  pagination.clear();
  pagination.showLoader();
  fetchBoardgames(0).then((pageCount) => {
    pagination.hideLoader();
    pagination.updateContext(0, pageCount);
  });
}

/*
 * Fetch boardgames for a specific page
 */
async function fetchBoardgames(pageIndex) {
  return fetch(
    `/api/v1/boardgames?limit=${itemLimit}&page=${pageIndex}&search=${boardgameSearchBar.value}`,
  ).then(async (response) => {
    const boardgames = await response.json();
    boardgames.forEach((boardgame) => {
      addBoardgame(boardgame, pageIndex);
    });
    modifyAvailabilityButtons();
    return response.headers.get("pagination-count");
  });
}

/*
 * Add boardgame to the item list
 */
function addBoardgame(boardgame, pageIndex) {
  let button_availability = boardgame.available ? "available" : "unavailable";
  let button_availability_not = boardgame.available
    ? "unavailable"
    : "available";

  boardgameContainer.insertAdjacentHTML(
    "beforeend",
    `
  <li class="boardgame flex-bar pagination-item hidden" pagination-index="${pageIndex}">
      <div class="flex-start">
          <h4>${boardgame.title}</h4>
          <p id="uid" onclick="copyUid(this)">
          ${boardgame.uid}<span class="tooltip">Copied!</span>
          </p>
      </div>
      <div class="flex-end">
        <form action="/admin/available/${boardgame.uid}" method="post">
          <button type="submit" class="slider availability" slider-hover-content="${button_availability_not}">${button_availability}</button>
        </form>
        <form action="/admin/edit/${boardgame.uid}" method="get">
          <button type="submit" id="edit">
            <i class="fas fa-edit fa-fw"></i> edit
          </button>
        </form>
        <form action="/admin/delete/${boardgame.uid}" method="post">
          <button type="submit" class="delete" onclick="return confirm('Delete boardgame?');">
            <i class="fas fa-trash fa-fw"></i> delete
          </button>
        </form>
      </div>
  </li>
  `,
  );
}

/*
 * Switch the boardgame's availability
 */
function setBoardgameAvailability(button) {
  let endpoint = button.parentElement.action;
  fetch(endpoint, { method: "POST" }).then(async (response) => {
    if (response.status != 200) {
      alert("Error: Unable to modify boardgame");
      return;
    }
    let availability = button.getAttribute("slider-hover-content");
    if (availability == "available") {
      button.setAttribute("slider-hover-content", "unavailable");
      button.innerText = "available";
    } else {
      button.setAttribute("slider-hover-content", "available");
      button.innerText = "unavailable";
    }
  });
}

/*
 * Copy selected uid and show tooltip
 */
function copyBoardgameUid(element) {
  navigator.clipboard.writeText(element.innerText).then(
    function () {
      element.firstElementChild.classList.toggle("show");
      setTimeout(() => {
        element.firstElementChild.classList.toggle("show");
      }, 1000);
    },
    function (err) {
      console.error("BGCat: Could not copy text ", err);
    },
  );
}
