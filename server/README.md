
# API

## Endpoints

All requests go to a specific versioned endpoint. For this prototype it's just version 1 of the api.
* https://localhost:8080/api/{version}/

Projects are scoped by user only. A real implementation feature would be to allow sharing projects - this is out of scope for this prototype project.
* Create a new project:
    POST https://localhost:8080/api/{version}/user/{userid}/project
* Get info about a project:
    GET https://localhost:8080/api/{version}/user/{userid}/project/{projectid}
* Update info about project:
    PATCH https://localhost:8080/api/{version}/user/{userid}/project/{projectid}
* Delete project
    DELETE https://localhost:8080/api/{version}/user/{userid}/project/{projectid}

Floorplan endpoints are scoped by project.
    * Create a new floorplan
        POST https://localhost:8080/api/{version}/user/{userid}/project/{projectid}/floorplan
    * Get info about a floorplan:
        GET https://localhost:8080/api/{version}/user/{userid}/project/{projectid}/floorplan/{floorplanid}
    * Update info about floorplan:
        PATCH https://localhost:8080/api/{version}/user/{userid}/project/{projectid}/floorplan/{floorplanid}
    * Delete floorplan:
        DELETE https://localhost:8080/api/{version}/user/{userid}/project/{projectid}/floorplan/{floorplanid}

# TODO

[ ] Add info about database and schema
[ ] Make detailed request/response info for the endpoints
[ ] Add info about authentication model
[ ] Add info about authorization model
[ ] Add info about project dependencies
[ ] Add info about project scope
[ ] Add info about known limitations
[ ] Add info about how this can scale
