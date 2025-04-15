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
      this.updateContext(currentPage, totalPages);
      this.registerButtons();
    } catch (err) {
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

  showLoader() {
    this.loader.classList.remove("hidden");
  }

  hideLoader() {
    this.loader.classList.add("hidden");
  }

  /*
   * Change the context of pagination
   * (i.e. when the list of items has changed by means of filters or search)
   */
  updateContext(currentPage, totalPages) {
    this.currentPage = Number(currentPage);
    this.pagesFetched = [];
    this.pagesFetched.push(Number(currentPage));
    this.totalPages = Number(totalPages);

    this.resetNumberButtons();
    this.resetVisibleButtons();
    this.resetActiveNumberButtons();
    this.resetActiveItems();
    this.registerNumberButtons();
  }

  /*
   * Register pagination for the numbered buttons
   */
  registerNumberButtons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.removeAttribute("href");
      button.addEventListener("click", () => {
        this.changePageIndex(Number(button.getAttribute("pagination-index")));
      });
    });
  }

  /*
   * Register pagination for the first,prev,next and last buttons
   */
  registerButtons() {
    this.buttons.forEach((button) => {
      button.removeAttribute("href");
    });

    this.buttonFirst.addEventListener("click", () => {
      this.changePageIndex(0);
    });

    this.buttonPrev.addEventListener("click", () => {
      if (this.currentPage > 0) {
        this.changePageIndex(this.currentPage - 1);
      }
    });

    this.buttonNext.addEventListener("click", () => {
      if (this.currentPage < this.totalPages - 1) {
        this.changePageIndex(this.currentPage + 1);
      }
    });

    this.buttonLast.addEventListener("click", () => {
      this.changePageIndex(this.totalPages - 1);
    });
  }

  /*
   * Change the active page
   * If the page is new => Fetch new page
   */
  changePageIndex(pageIndex) {
    if (!this.pagesFetched.includes(pageIndex)) {
      this.showLoader();
      this.itemFetchCallback(pageIndex)
        .then(() => {
          this.pagesFetched.push(pageIndex);
          this.activatePage(pageIndex);
          this.hideLoader();
        })
        .catch(console.error);
    } else {
      this.activatePage(pageIndex);
    }
  }

  /*
   * Activate a certain page
   */
  activatePage(pageIndex) {
    this.currentPage = pageIndex;
    this.resetNumberButtons();
    this.resetActiveNumberButtons();
    this.resetActiveItems();
    this.registerNumberButtons();
  }

  /*
   * Reset the visibility for the items
   */
  resetActiveItems() {
    [...this.itemsLive].forEach((item) => {
      item.classList.add("hidden");
      if (Number(item.getAttribute("pagination-index")) === this.currentPage) {
        item.classList.remove("hidden");
      }
    });
  }

  resetNumberButtons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.remove();
    });

    let page_range_start = this.currentPage <= 2 ? 0 : this.currentPage - 2;
    let page_range_end =
      this.currentPage >= this.totalPages - 3
        ? this.totalPages
        : this.currentPage + 3;

    for (var i = page_range_start; i < page_range_end; i++) {
      this.numberButtonContainer.insertAdjacentHTML(
        "beforeend",
        `
      <a class="pagination-number" pagination-index="${i}" title="Page ${i}" aria-label="page ${i}">${i}</a>
      `,
      );
    }
  }

  /*
   * Reset the active/inactive numbered buttons
   */
  resetActiveNumberButtons() {
    [...this.numberButtonsLive].forEach((button) => {
      button.classList.remove("active");
      if (
        Number(button.getAttribute("pagination-index")) === this.currentPage
      ) {
        button.classList.add("active");
      }
    });
  }

  /*
   * Reset the visibility of the first,prev,next and last buttons
   */
  resetVisibleButtons() {
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
