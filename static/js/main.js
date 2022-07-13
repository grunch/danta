const App = {
  endpoint: "/api",
  interval: null,
  server: "http://localhost:3000",
};

App.init = () => {
  $("#attendee-form").collapse("show");
  $("#form").on("submit", App.submit);
};

App.submit = async (e) => {
  try {
    e.preventDefault();
    const name = $("#name").val();
    const lastname = $("#lastname").val();
    const email = $("#email").val();

    const response = await App.makeRequest({
      api: "invoice",
      post: { name, lastname, email },
    });

    if (!response) console.error("Error getting data!");
    if (response.success) {
      $("#attendee-form").collapse("hide");
      $("#invoice").collapse("show");
      $("#invoice-text").text(response.request);
      $("#invoice-memo").text(response.description);
      $("#invoice-amount").text(`${response.amount} `);
      const qrCode = App.qrCode(response.request.toUpperCase(), 400);
      $("#qr-code").html(qrCode);
      App.interval = setInterval(App.waitPayment, 2000, response.hash);
    }
  } catch (error) {
    console.log(error.responseJSON);
  }
};

App.waitPayment = async (hash) => {
  const response = await App.makeRequest({
    api: `invoice/${hash}`,
  });
  if (response.success && response.paid) {
    clearInterval(App.interval);
    App.interval = null;
    $("#invoice").collapse("hide");
    const url = `${App.server}${App.endpoint}/verify/${response.preimage}`;
    const qrCode = App.qrCode(url, 400);
    $(".preimage-qr-code").html(qrCode);
    $("#success-box").collapse("show");
    setTimeout(App.getBalance, 2000);
  }
};

/** Get qr code
  {
    text: <String>
  }

  @returns
  <QR Code Img Object>
*/
App.qrCode = (text) => {
  const back = "rgb(250, 250, 250)";
  const rounded = 100;
  const size = 300;

  const qr = kjua({ back, rounded, size, text });

  $(qr).css({ height: "auto", "max-width": "200px", width: "100%" });

  return qr;
};

App.makeRequest = ({ api, post }) => {
  const type = !post ? "GET" : "POST";
  const data = !!post ? JSON.stringify(post) : null;
  return $.ajax(`${App.endpoint}/${api}`, {
    type,
    data,
    contentType: "application/json",
    dataType: "json",
  });
};

$(() => App.init());
