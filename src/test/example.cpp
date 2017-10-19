#include "catch.hpp"
SCENARIO("Test Scenario") {
     GIVEN("an int") {
         int x = 5;
         THEN("check that it is 5") {
             CHECK( x == 5 );
         }
     }
}
