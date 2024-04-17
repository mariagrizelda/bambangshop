# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [ ] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [x] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [x] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [x] Commit: `Implement publish function in Program service and Program controller.`
    -   [x] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [x] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
In the traditional Observer pattern, the Subscriber role is often 
represented as an interface, fostering loose coupling between the subject 
and its observers. However, in the specific scenario of BambangShop, it 
seems that a single Model struct could adequately serve the purpose. 
Rust's ownership system offers a degree of loose coupling without the 
explicit need for interfaces. If all observers share similar update logic, 
consolidating them within a single Model struct, responsible for both data 
management and notification dissemination, could streamline the design. 
Nonetheless, interfaces might prove beneficial if there's a necessity for 
diverse update behaviors or if the application anticipates dynamically 
adding observers during runtime.

Regarding the management of unique identifiers such as the id in Program 
and the url in Subscriber, the choice between using a simple Vec or 
DashMap is pivotal. While a Vec could suffice for maintaining a list, 
DashMap presents several advantages, particularly in ensuring uniqueness 
and expediting retrieval. DashMap's key-based lookup mechanism inherently 
guards against accidental duplicates, minimizing the risk of 
inconsistencies or errors that may arise from inadvertent replication.

In the Rust programming ecosystem, the stringent compiler constraints 
inherently enforce the development of thread-safe applications. With 
respect to the List of Subscribers (SUBSCRIBERS) static variable, although 
a Singleton pattern might offer the allure of a unified access point, 
opting for DashMap for ensuring thread safety seems judicious. DashMap's 
design explicitly caters to concurrent access, obviating the need for 
additional safeguards like mutexes typically associated with Singleton 
implementations. Given the existing utilization of DashMap for managing 
the subscriber list and its inherent suitability for concurrent 
environments, maintaining its usage is prudent to uphold the thread-safe 
integrity of the application.

#### Reflection Publisher-2

1.Separating Service and Repository from Model is crucial to maintain, scale, and test code effectively, aligning with core design principles such as Single Responsibility Principle (SRP), Separation of 
Concerns (SoC), and Don't Repeat Yourself (DRY). This division promotes cleaner code organization, simplifies maintenance, and facilitates accommodating changes as the system evolves and becomes more 
intricate.

2.Depending solely on the Model would likely result in excessive responsibilities, increasing complexity and undermining maintainability and modularity. This approach would burden Models with both 
business logic and data access tasks, leading to a tangled codebase that's challenging to extend or modify.

3.Postman significantly streamlines the API endpoint testing process. It simplifies the execution of calls to these endpoints, enabling users to verify program functionality efficiently. Particularly 
valuable is Postman's feature that allows organizing endpoints into collections for convenient testing, and even automating these tests, thereby boosting productivity. #### Reflection Publisher-3

#### Reflection Publisher-3
1.Which variant of the pattern is applied?: The implemented approach adopts the push variant of the observer pattern. In this setup, the NotificationService acts as the subject, actively dispatching 
updates to its observers (Subscribers) whenever there's a change.

2.What advantages does the observer pattern offer in this guide?: Employing the pull method within the observer pattern simplifies the system by relieving the subject of the burden of tracking its 
observers. Furthermore, this strategy bolsters scalability by allowing observers to fetch updates only when needed, thereby freeing the subject from the obligation to broadcast updates to all observers 
simultaneously.

3.What are the consequences of omitting multi-threading?: Absence of multi-threading could lead to noticeable slowdowns in notification delivery, particularly when numerous users access the application 
concurrently. Furthermore, the full potential of available hardware resources may remain untapped, potentially resulting in inefficiencies.
