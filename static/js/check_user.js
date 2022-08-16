const App = {
  endpoint: "/api",
  interval: null,
  server: "https://lightninghackday.info",
};

App.init = () => {
  $(".attendee-form").collapse("show");
  $("#no-paid").hide();
  $("#form").on("submit", App.submit);
};

App.submit = async (e) => {
  try {
    e.preventDefault();
    const email = $("#email").val();

    const response = await App.makeRequest({
      api: "verify",
      data: { email_str: email },
    });

    if (!response) console.error("Error getting data!");

    if (response.paid) {
      $(".attendee-form").collapse("hide");
      $("#pdffile").attr("href", `/files/${response.preimage}.pdf`);
      $("#success-box").collapse("show");
    } else {
      $("#no-paid").show();
    }
  } catch (error) {
    console.log(error.responseJSON);
  }
};

App.makeRequest = ({ api, data }) => {
  return $.ajax(`${App.endpoint}/${api}`, {
    type: "get",
    data,
  });
};

$(() => App.init());
