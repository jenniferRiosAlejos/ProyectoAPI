<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Agencias - 200</name>
   <tag></tag>
   <elementGuidId>60dc84cc-1774-4740-a1e9-03b020ad18df</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDU1NDgyODksIm5vbWJyZSI6IjMxOTkyMjgifQ.a6ajlrd2huSOSc_pWaZBPRuVuwHg-HQ7xU8T_OBn7QjdqDpLFGlnAMOC57nTFkoEyt1LROGFAkKISH5XZUif-w</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;country_id\&quot;: \&quot;PE\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>0d16010f-cf3d-4981-a4c6-18bfa45f4af3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDYxNzUyOTcsImF1ZGllbmNlIjoiYWdlbmNpZXMiLCJub21icmUiOiIzMTk5MjI4In0.5b234XQq6G0CGdMeyfxa1rkgjPN3IF23q57sKAkfG1Ia4Eb0iELuy7vbJaLVSh_nUqfrW4jRULia8Q26nopHUw</value>
      <webElementGuid>ef2e5e64-7793-483c-8729-5c06dbdd02f1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-apimp.olvacourier.com/agencies</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
