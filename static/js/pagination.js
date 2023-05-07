"use strict";

class Pagination {
  currentPage = Number(0);
  totalPages = Number(0);
  pagesFetched = [];

  itemFetchCallback = null;

  loader = document.getElementById("pagination-loader");
  buttons = document.querySelectorAll(".pagination-button");
  buttonFirst = document.getElementById("pagination-first");
  buttonPrev = document.getElementById("pagination-prev");
  buttonNext = document.getElementById("pagination-next");
  buttonLast = document.getElementById("pagination-last");
  numberButtonContainer = document.getElementById("pagination-numbers");
  numberButtonsLive = document.getElementsByClassName("pagination-number");
  itemsLive = document.getElementsByClassName("pagination-item");

  constructor(currentPage, totalPages, itemFetchCallback) {
    try {
      this.itemFetchCallback = itemFetchCallback;
      this.update_context(currentPage, totalPages);
      this.register_buttons();
    } catch(err) {
      console.error(`bgcat: unable to register pagination ${err}`);
    }
    console.log("bgcat: Registered pagination");
  }

  clear() {
    [...this.itemsLive].forEach((item) => {
      item.remove();
    });
    [...this.numberButtonsLive].forEach((button) => {
      button.remove();
    });
    this.buttons.forEach((button) => {
      button.classList.add("hidden");
    });
  }

  show_loader() {
    this.loader.classList.remove("hidden");
  }

  hide_loader() {
    this.loader.classList.add("hidden");
  }

  /*
   * Change the context of pagination
   * (i.e. when the list of items has changed by means of filters or search)
   */
  update_context(currentPage, totalPages) {
    this.currentPage = Number(currentPage);
    this.pagesFetched = [];
    this.pagesFetched.push(Number(currentPage));
    this.totalPages = Number(totalPages);

    this.reset_number_buttons();
    this.reset_visible_buttons();
    this.reset_active_number_buttons();
    this.reset_active_items();
    this.register_number_buttons();
  }

  /*
   * Register pagination for the numbered buttons
   */
  register_number_buttons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.removeAttribute("href");
      button.addEventListener("click", () => {
        this.change_page_index(Number(button.getAttribute("pagination-index")));
      });
    });
  }

  /*
   * Register pagination for the first,prev,next and last buttons
   */
  register_buttons() {
    this.buttons.forEach((button) => {
      button.removeAttribute("href");
    });

    this.buttonFirst.addEventListener("click", () => {
      this.change_page_index(0);
    });

    this.buttonPrev.addEventListener("click", () => {
      if (this.currentPage > 0) {
        this.change_page_index(this.currentPage - 1);
      }
    });

    this.buttonNext.addEventListener("click", () => {
      if (this.currentPage < this.totalPages - 1) {
        this.change_page_index(this.currentPage + 1);
      }
    });

    this.buttonLast.addEventListener("click", () => {
      this.change_page_index(this.totalPages - 1);
    });
  }

  /*
   * Change the active page
   * If the page is new => Fetch new page
   */
  change_page_index(pageIndex) {
    if (!this.pagesFetched.includes(pageIndex)) {
      this.show_loader();
      this.itemFetchCallback(pageIndex)
        .then(() => {
          this.pagesFetched.push(pageIndex);
          this.activate_page(pageIndex);
          this.hide_loader();
        })
        .catch(console.error);
    } else {
      this.activate_page(pageIndex);
    }
  }

  /*
   * Activate a certain page
   */
  activate_page(pageIndex) {
    this.currentPage = pageIndex;
    this.reset_active_items();
    this.reset_active_number_buttons();
  }

  /*
   * Reset the visibility for the items
   */
  reset_active_items() {
    [...this.itemsLive].forEach((item) => {
      item.classList.add("hidden");
      if(Number(item.getAttribute("pagination-index")) === this.currentPage){
        item.classList.remove("hidden");
      }
    });
  }

  reset_number_buttons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.remove();
    });
    for (var i=0; i<this.totalPages; i++) {
      this.numberButtonContainer.insertAdjacentHTML("beforeend",`
      <a class="pagination-number" pagination-index="${i}" title="Page ${i}" aria-label="page ${i}">${i}</a>
      `);
    }
  }

  /*
   * Reset the active/inactive numbered buttons
   */
  reset_active_number_buttons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.classList.remove("active");
      if(Number(button.getAttribute("pagination-index")) === this.currentPage){
        button.classList.add("active");
      }
    });
  }

  /*
   * Reset the visibility of the first,prev,next and last buttons
   */
  reset_visible_buttons() {
    if (this.totalPages <= 1) {
      this.buttons.forEach((button) => {
        button.classList.add("hidden");
      });
    } else {
      this.buttons.forEach((button) => {
        button.classList.remove("hidden");
      });
    }
  }
}