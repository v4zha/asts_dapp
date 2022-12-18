import { Html, Head, Main, NextScript } from 'next/document'

export default function Document() {
  return (
    <Html lang="en">
      <Head>
        <link rel="stylesheet" href="/assets/vendors/mdi/css/materialdesignicons.min.css" />
        <link rel="stylesheet" href="/assets/vendors/css/vendor.bundle.base.css" />
        <link rel="stylesheet" href="/assets/vendors/jvectormap/jquery-jvectormap.css" />
        <link rel="stylesheet" href="/assets/vendors/flag-icon-css/css/flag-icon.min.css" />
        <link rel="stylesheet" href="/assets/vendors/owl-carousel-2/owl.carousel.min.css" />
        <link rel="stylesheet" href="/assets/vendors/owl-carousel-2/owl.theme.default.min.css" />
        <link rel="stylesheet" href="/assets/css/style.css" />
        <link rel="shortcut icon" href="/assets/images/favicon.png" />
        <link rel="stylesheet" href="../../assets/vendors/select2-bootstrap-theme/select2-bootstrap.min.css"></link>
      </Head>
      <body>
        <Main />
        <NextScript />
        <script src="/assets/vendors/js/vendor.bundle.base.js"></script>
        {/* <script src="/assets/vendors/chart.js/Chart.min.js"></script>
        <script src="/assets/vendors/progressbar.js/progressbar.min.js"></script>
        <script src="/assets/vendors/jvectormap/jquery-jvectormap.min.js"></script>
        <script src="/assets/vendors/jvectormap/jquery-jvectormap-world-mill-en.js"></script>
        <script src="/assets/vendors/owl-carousel-2/owl.carousel.min.js"></script>
        <script src="/assets/js/off-canvas.js"></script>
        <script src="/assets/js/hoverable-collapse.js"></script>
        <script src="/assets/js/misc.js"></script>
        <script src="/assets/js/settings.js"></script>
        <script src="/assets/js/todolist.js"></script>
        <script src="/assets/js/dashboard.js"></script> */}
      </body>
    </Html>
  )
}
